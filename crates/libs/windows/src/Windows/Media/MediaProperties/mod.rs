#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct AudioEncodingProperties(::windows::core::IUnknown);
impl AudioEncodingProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioEncodingProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitrate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Bitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetChannelCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetChannelCount)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn ChannelCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChannelCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetSampleRate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSampleRate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SampleRate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SampleRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetBitsPerSample(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitsPerSample)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn BitsPerSample(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BitsPerSample)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn IsSpatial(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioEncodingProperties2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSpatial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Copy(&self) -> ::windows::core::Result<AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioEncodingProperties3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Copy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateAac(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAac)(::core::mem::transmute_copy(this), samplerate, channelcount, bitrate, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateAacAdts(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAacAdts)(::core::mem::transmute_copy(this), samplerate, channelcount, bitrate, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateMp3(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMp3)(::core::mem::transmute_copy(this), samplerate, channelcount, bitrate, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreatePcm(samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePcm)(::core::mem::transmute_copy(this), samplerate, channelcount, bitspersample, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateWma(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWma)(::core::mem::transmute_copy(this), samplerate, channelcount, bitrate, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateAlac(samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAlac)(::core::mem::transmute_copy(this), samplerate, channelcount, bitspersample, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateFlac(samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFlac)(::core::mem::transmute_copy(this), samplerate, channelcount, bitspersample, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetFormatUserData(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioEncodingPropertiesWithFormatUserData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFormatUserData)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn GetFormatUserData(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioEncodingPropertiesWithFormatUserData>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetFormatUserData)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<MediaPropertySet> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetSubtype<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSubtype)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtype)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IAudioEncodingPropertiesStatics<R, F: FnOnce(&IAudioEncodingPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioEncodingProperties, IAudioEncodingPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAudioEncodingPropertiesStatics2<R, F: FnOnce(&IAudioEncodingPropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioEncodingProperties, IAudioEncodingPropertiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioEncodingProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioEncodingProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioEncodingProperties {}
impl ::core::fmt::Debug for AudioEncodingProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioEncodingProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioEncodingProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.AudioEncodingProperties;{62bc7a16-005c-4b3b-8a0b-0a090e9687f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioEncodingProperties {
    type Vtable = IAudioEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = <IAudioEncodingProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.AudioEncodingProperties";
}
impl ::core::convert::From<AudioEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: AudioEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: &AudioEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: AudioEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: &AudioEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioEncodingProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioEncodingProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaEncodingProperties> for AudioEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaEncodingProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaEncodingProperties> for &AudioEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaEncodingProperties> {
        ::core::convert::TryInto::<IMediaEncodingProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioEncodingProperties {}
unsafe impl ::core::marker::Sync for AudioEncodingProperties {}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioEncodingQuality(pub i32);
impl AudioEncodingQuality {
    pub const Auto: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Medium: Self = Self(2i32);
    pub const Low: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioEncodingQuality {}
impl ::core::clone::Clone for AudioEncodingQuality {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioEncodingQuality {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioEncodingQuality {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioEncodingQuality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioEncodingQuality").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioEncodingQuality {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.AudioEncodingQuality;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct ContainerEncodingProperties(::windows::core::IUnknown);
impl ContainerEncodingProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContainerEncodingProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Copy(&self) -> ::windows::core::Result<ContainerEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IContainerEncodingProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Copy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContainerEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<MediaPropertySet> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetSubtype<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSubtype)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtype)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ContainerEncodingProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContainerEncodingProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContainerEncodingProperties {}
impl ::core::fmt::Debug for ContainerEncodingProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContainerEncodingProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContainerEncodingProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.ContainerEncodingProperties;{59ac2a57-b32a-479e-8a61-4b7f2e9e7ea0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContainerEncodingProperties {
    type Vtable = IContainerEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = <IContainerEncodingProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContainerEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.ContainerEncodingProperties";
}
impl ::core::convert::From<ContainerEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: ContainerEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContainerEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: &ContainerEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContainerEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContainerEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContainerEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: ContainerEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContainerEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: &ContainerEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContainerEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContainerEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContainerEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: ContainerEncodingProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContainerEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContainerEncodingProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaEncodingProperties> for ContainerEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaEncodingProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaEncodingProperties> for &ContainerEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaEncodingProperties> {
        ::core::convert::TryInto::<IMediaEncodingProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContainerEncodingProperties {}
unsafe impl ::core::marker::Sync for ContainerEncodingProperties {}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
pub struct H264ProfileIds {}
impl H264ProfileIds {
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn ConstrainedBaseline() -> ::windows::core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConstrainedBaseline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Baseline() -> ::windows::core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Baseline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Extended() -> ::windows::core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Extended)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Main() -> ::windows::core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Main)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn High() -> ::windows::core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).High)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn High10() -> ::windows::core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).High10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn High422() -> ::windows::core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).High422)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn High444() -> ::windows::core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).High444)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn StereoHigh() -> ::windows::core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StereoHigh)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn MultiviewHigh() -> ::windows::core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MultiviewHigh)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IH264ProfileIdsStatics<R, F: FnOnce(&IH264ProfileIdsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<H264ProfileIds, IH264ProfileIdsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for H264ProfileIds {
    const NAME: &'static str = "Windows.Media.MediaProperties.H264ProfileIds";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEncodingProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioEncodingProperties {
    type Vtable = IAudioEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62bc7a16_005c_4b3b_8a0b_0a090e9687f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetSampleRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub SampleRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetBitsPerSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub BitsPerSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEncodingProperties2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioEncodingProperties2 {
    type Vtable = IAudioEncodingProperties2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc45d54da_80bd_4c23_80d5_72d4a181e894);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingProperties2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSpatial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEncodingProperties3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioEncodingProperties3 {
    type Vtable = IAudioEncodingProperties3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87600341_748c_4f8d_b0fd_10caf08ff087);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingProperties3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEncodingPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioEncodingPropertiesStatics {
    type Vtable = IAudioEncodingPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cad332c_ebe9_4527_b36d_e42a13cf38db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateAac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateAacAdts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateMp3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreatePcm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEncodingPropertiesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioEncodingPropertiesStatics2 {
    type Vtable = IAudioEncodingPropertiesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7489316f_77a0_433d_8ed5_4040280e8665);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateAlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEncodingPropertiesWithFormatUserData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioEncodingPropertiesWithFormatUserData {
    type Vtable = IAudioEncodingPropertiesWithFormatUserData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98f10d79_13ea_49ff_be70_2673db69702c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesWithFormatUserData_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContainerEncodingProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContainerEncodingProperties {
    type Vtable = IContainerEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59ac2a57_b32a_479e_8a61_4b7f2e9e7ea0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContainerEncodingProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContainerEncodingProperties2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContainerEncodingProperties2 {
    type Vtable = IContainerEncodingProperties2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb272c029_ae26_4819_baad_ad7a49b0a876);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContainerEncodingProperties2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IH264ProfileIdsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IH264ProfileIdsStatics {
    type Vtable = IH264ProfileIdsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38654ca7_846a_4f97_a2e5_c3a15bbf70fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IH264ProfileIdsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ConstrainedBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Baseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Extended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Main: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub High: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub High10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub High422: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub High444: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub StereoHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MultiviewHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageEncodingProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageEncodingProperties {
    type Vtable = IImageEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78625635_f331_4189_b1c3_b48d5ae034f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageEncodingProperties2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageEncodingProperties2 {
    type Vtable = IImageEncodingProperties2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc854a2df_c923_469b_ac8e_6a9f3c1cd9e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingProperties2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageEncodingPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageEncodingPropertiesStatics {
    type Vtable = IImageEncodingPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x257c68dc_8b99_439e_aa59_913a36161297);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateJpeg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreatePng: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateJpegXR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageEncodingPropertiesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageEncodingPropertiesStatics2 {
    type Vtable = IImageEncodingPropertiesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6c25b29_3824_46b0_956e_501329e1be3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateUncompressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: MediaPixelFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateBmp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageEncodingPropertiesStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageEncodingPropertiesStatics3 {
    type Vtable = IImageEncodingPropertiesStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48f4814d_a2ff_48dc_8ea0_e90680663656);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateHeif: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingProfile(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingProfile {
    type Vtable = IMediaEncodingProfile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7dbf5a8_1db9_4783_876b_3dfe12acfdb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfile_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Audio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Video: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingProfile2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingProfile2 {
    type Vtable = IMediaEncodingProfile2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x349b3e0a_4035_488e_9877_85632865ed10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfile2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub SetAudioTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    SetAudioTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub GetAudioTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    GetAudioTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub SetVideoTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    SetVideoTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub GetVideoTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    GetVideoTracks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingProfile3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingProfile3 {
    type Vtable = IMediaEncodingProfile3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba6ebe88_7570_4e69_accf_5611ad015f88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfile3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub SetTimedMetadataTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    SetTimedMetadataTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub GetTimedMetadataTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    GetTimedMetadataTracks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingProfileStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingProfileStatics {
    type Vtable = IMediaEncodingProfileStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x197f352c_2ede_4a45_a896_817a4854f8fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateM4a: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateMp3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateMp4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWmv: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFromFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFromStreamAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingProfileStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingProfileStatics2 {
    type Vtable = IMediaEncodingProfileStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce8de74f_6af4_4288_8fe2_79adf1f79a43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateWav: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateAvi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingProfileStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingProfileStatics3 {
    type Vtable = IMediaEncodingProfileStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90dac5aa_cf76_4294_a9ed_1a1420f51f6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateAlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateHevc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct IMediaEncodingProperties(::windows::core::IUnknown);
impl IMediaEncodingProperties {
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<MediaPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetSubtype<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubtype)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtype)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IMediaEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: IMediaEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: &IMediaEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: IMediaEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: &IMediaEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMediaEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaEncodingProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaEncodingProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaEncodingProperties {}
impl ::core::fmt::Debug for IMediaEncodingProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaEncodingProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaEncodingProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b4002af6-acd4-4e5a-a24b-5d7498a8b8c4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IMediaEncodingProperties {
    type Vtable = IMediaEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4002af6_acd4_4e5a_a24b_5d7498a8b8c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSubtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Subtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingSubtypesStatics {
    type Vtable = IMediaEncodingSubtypesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37b6580e_a171_4464_ba5a_53189e48c1c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Aac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AacAdts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ac3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AmrNb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AmrWb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Argb32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Asf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Avi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Bgra8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Bmp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Eac3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Float: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Gif: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub H263: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub H264: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub H264Es: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Hevc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HevcEs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Iyuv: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Jpeg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub JpegXr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Mjpg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Mpeg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Mpeg1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Mpeg2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Mp3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Mpeg4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Nv12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Pcm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Png: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Rgb24: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Rgb32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Tiff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Wave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Wma8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Wma9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Wmv3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Wvc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Yuy2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Yv12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingSubtypesStatics2 {
    type Vtable = IMediaEncodingSubtypesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b7cd23d_42ff_4d33_8531_0626bee4b52d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Vp9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub L8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub L16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub D16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingSubtypesStatics3 {
    type Vtable = IMediaEncodingSubtypesStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba2414e4_883d_464e_a44f_097da08ef7ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Alac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Flac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingSubtypesStatics4 {
    type Vtable = IMediaEncodingSubtypesStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddece58a_3949_4644_8a2c_59ef02c642fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub P010: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingSubtypesStatics5 {
    type Vtable = IMediaEncodingSubtypesStatics5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ad4a007_ffce_4760_9828_5d0c99637e6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Heif: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaEncodingSubtypesStatics6 {
    type Vtable = IMediaEncodingSubtypesStatics6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1252973_a984_5912_93bb_54e7e569e053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Pgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Srt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ssa: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VobSub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaRatio(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaRatio {
    type Vtable = IMediaRatio_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2d0fee5_8929_401d_ac78_7d357e378163);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRatio_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetNumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Numerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDenominator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Denominator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMpeg2ProfileIdsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMpeg2ProfileIdsStatics {
    type Vtable = IMpeg2ProfileIdsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa461ff85_e57a_4128_9b21_d5331b04235c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMpeg2ProfileIdsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Simple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Main: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SignalNoiseRatioScalable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SpatiallyScalable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub High: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataEncodingProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedMetadataEncodingProperties {
    type Vtable = ITimedMetadataEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51cd30d3_d690_4cfa_97f4_4a398e9db420);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataEncodingProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataEncodingPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedMetadataEncodingPropertiesStatics {
    type Vtable = ITimedMetadataEncodingPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6629bb67_6e55_5643_89a0_7a7e8d85b52c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataEncodingPropertiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreatePgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSrt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSsa: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateVobSub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEncodingProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoEncodingProperties {
    type Vtable = IVideoEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76ee6c9a_37c2_4f2a_880a_1282bbb4373d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub FrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PixelAspectRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEncodingProperties2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoEncodingProperties2 {
    type Vtable = IVideoEncodingProperties2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf743a1ef_d465_4290_a94b_ef0f1528f8e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetProfileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub ProfileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEncodingProperties3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoEncodingProperties3 {
    type Vtable = IVideoEncodingProperties3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x386bcdc4_873a_479f_b3eb_56c1fcbec6d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StereoscopicVideoPackingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StereoscopicVideoPackingMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEncodingProperties4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoEncodingProperties4 {
    type Vtable = IVideoEncodingProperties4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x724ef014_c10c_40f2_9d72_3ee13b45fa8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SphericalVideoFrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SphericalVideoFrameFormat) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEncodingProperties5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoEncodingProperties5 {
    type Vtable = IVideoEncodingProperties5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4959080f_272f_4ece_a4df_c0ccdb33d840);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEncodingPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoEncodingPropertiesStatics {
    type Vtable = IVideoEncodingPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ce14d44_1dc5_43db_9f38_ebebf90152cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingPropertiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateH264: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateMpeg2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateUncompressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, width: u32, height: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEncodingPropertiesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoEncodingPropertiesStatics2 {
    type Vtable = IVideoEncodingPropertiesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf1ebd5d_49fe_4d00_b59a_cfa4dfc51944);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingPropertiesStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateHevc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct ImageEncodingProperties(::windows::core::IUnknown);
impl ImageEncodingProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ImageEncodingProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWidth)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Copy(&self) -> ::windows::core::Result<ImageEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IImageEncodingProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Copy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateJpeg() -> ::windows::core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateJpeg)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreatePng() -> ::windows::core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePng)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateJpegXR() -> ::windows::core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateJpegXR)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateUncompressed(format: MediaPixelFormat) -> ::windows::core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateUncompressed)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateBmp() -> ::windows::core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateBmp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateHeif() -> ::windows::core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateHeif)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<MediaPropertySet> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetSubtype<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSubtype)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtype)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IImageEncodingPropertiesStatics<R, F: FnOnce(&IImageEncodingPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ImageEncodingProperties, IImageEncodingPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IImageEncodingPropertiesStatics2<R, F: FnOnce(&IImageEncodingPropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ImageEncodingProperties, IImageEncodingPropertiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IImageEncodingPropertiesStatics3<R, F: FnOnce(&IImageEncodingPropertiesStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ImageEncodingProperties, IImageEncodingPropertiesStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ImageEncodingProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageEncodingProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageEncodingProperties {}
impl ::core::fmt::Debug for ImageEncodingProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageEncodingProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ImageEncodingProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.ImageEncodingProperties;{78625635-f331-4189-b1c3-b48d5ae034f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ImageEncodingProperties {
    type Vtable = IImageEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = <IImageEncodingProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ImageEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.ImageEncodingProperties";
}
impl ::core::convert::From<ImageEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: ImageEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: &ImageEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ImageEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ImageEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: ImageEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: &ImageEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ImageEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ImageEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ImageEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageEncodingProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageEncodingProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaEncodingProperties> for ImageEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaEncodingProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaEncodingProperties> for &ImageEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaEncodingProperties> {
        ::core::convert::TryInto::<IMediaEncodingProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ImageEncodingProperties {}
unsafe impl ::core::marker::Sync for ImageEncodingProperties {}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct MediaEncodingProfile(::windows::core::IUnknown);
impl MediaEncodingProfile {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaEncodingProfile, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetAudio<'a, Param0: ::windows::core::IntoParam<'a, AudioEncodingProperties>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudio)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Audio(&self) -> ::windows::core::Result<AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Audio)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetVideo<'a, Param0: ::windows::core::IntoParam<'a, VideoEncodingProperties>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVideo)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Video(&self) -> ::windows::core::Result<VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Video)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetContainer<'a, Param0: ::windows::core::IntoParam<'a, ContainerEncodingProperties>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContainer)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Container(&self) -> ::windows::core::Result<ContainerEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Container)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContainerEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn SetAudioTracks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::AudioStreamDescriptor>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioTracks)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::AudioStreamDescriptor>> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAudioTracks)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Core::AudioStreamDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn SetVideoTracks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::VideoStreamDescriptor>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoTracks)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetVideoTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::VideoStreamDescriptor>> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVideoTracks)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Core::VideoStreamDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn SetTimedMetadataTracks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataStreamDescriptor>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProfile3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTimedMetadataTracks)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetTimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::TimedMetadataStreamDescriptor>> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProfile3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTimedMetadataTracks)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Core::TimedMetadataStreamDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateM4a(quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateM4a)(::core::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateMp3(quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMp3)(::core::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateWma(quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWma)(::core::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateMp4(quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMp4)(::core::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateWmv(quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWmv)(::core::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFromFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromFileAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateFromStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(stream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromStreamAsync)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateWav(quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWav)(::core::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateAvi(quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAvi)(::core::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateAlac(quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAlac)(::core::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateFlac(quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFlac)(::core::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateHevc(quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateHevc)(::core::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaEncodingProfileStatics<R, F: FnOnce(&IMediaEncodingProfileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaEncodingProfile, IMediaEncodingProfileStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaEncodingProfileStatics2<R, F: FnOnce(&IMediaEncodingProfileStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaEncodingProfile, IMediaEncodingProfileStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaEncodingProfileStatics3<R, F: FnOnce(&IMediaEncodingProfileStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaEncodingProfile, IMediaEncodingProfileStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaEncodingProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaEncodingProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaEncodingProfile {}
impl ::core::fmt::Debug for MediaEncodingProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaEncodingProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaEncodingProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.MediaEncodingProfile;{e7dbf5a8-1db9-4783-876b-3dfe12acfdb3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaEncodingProfile {
    type Vtable = IMediaEncodingProfile_Vtbl;
    const IID: ::windows::core::GUID = <IMediaEncodingProfile as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaEncodingProfile {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaEncodingProfile";
}
impl ::core::convert::From<MediaEncodingProfile> for ::windows::core::IUnknown {
    fn from(value: MediaEncodingProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaEncodingProfile> for ::windows::core::IUnknown {
    fn from(value: &MediaEncodingProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaEncodingProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaEncodingProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaEncodingProfile> for ::windows::core::IInspectable {
    fn from(value: MediaEncodingProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaEncodingProfile> for ::windows::core::IInspectable {
    fn from(value: &MediaEncodingProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaEncodingProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaEncodingProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaEncodingProfile {}
unsafe impl ::core::marker::Sync for MediaEncodingProfile {}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
pub struct MediaEncodingSubtypes {}
impl MediaEncodingSubtypes {
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Aac() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Aac)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn AacAdts() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AacAdts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Ac3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ac3)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn AmrNb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AmrNb)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn AmrWb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AmrWb)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Argb32() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Argb32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Asf() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Asf)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Avi() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Avi)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Bgra8() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bgra8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Bmp() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bmp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Eac3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Eac3)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Float() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Float)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Gif() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gif)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn H263() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).H263)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn H264() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).H264)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn H264Es() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).H264Es)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Hevc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Hevc)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn HevcEs() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HevcEs)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Iyuv() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Iyuv)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Jpeg() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Jpeg)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn JpegXr() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).JpegXr)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Mjpg() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mjpg)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Mpeg() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mpeg)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Mpeg1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mpeg1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Mpeg2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mpeg2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Mp3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mp3)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Mpeg4() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mpeg4)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Nv12() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Nv12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Pcm() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pcm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Png() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Png)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Rgb24() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Rgb24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Rgb32() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Rgb32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Tiff() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tiff)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Wave() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Wave)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Wma8() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Wma8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Wma9() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Wma9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Wmv3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Wmv3)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Wvc1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Wvc1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Yuy2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Yuy2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Yv12() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Yv12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Vp9() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Vp9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn L8() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).L8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn L16() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).L16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn D16() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).D16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Alac() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics3(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Alac)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Flac() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics3(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Flac)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn P010() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).P010)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Heif() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics5(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Heif)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Pgs() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pgs)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Srt() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Srt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Ssa() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ssa)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn VobSub() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VobSub)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics<R, F: FnOnce(&IMediaEncodingSubtypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics2<R, F: FnOnce(&IMediaEncodingSubtypesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics3<R, F: FnOnce(&IMediaEncodingSubtypesStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics4<R, F: FnOnce(&IMediaEncodingSubtypesStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics5<R, F: FnOnce(&IMediaEncodingSubtypesStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics6<R, F: FnOnce(&IMediaEncodingSubtypesStatics6) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics6> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MediaEncodingSubtypes {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaEncodingSubtypes";
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaMirroringOptions(pub u32);
impl MediaMirroringOptions {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
}
impl ::core::marker::Copy for MediaMirroringOptions {}
impl ::core::clone::Clone for MediaMirroringOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaMirroringOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaMirroringOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaMirroringOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaMirroringOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MediaMirroringOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MediaMirroringOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MediaMirroringOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MediaMirroringOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MediaMirroringOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for MediaMirroringOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaMirroringOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaPixelFormat(pub i32);
impl MediaPixelFormat {
    pub const Nv12: Self = Self(0i32);
    pub const Bgra8: Self = Self(1i32);
    pub const P010: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPixelFormat {}
impl ::core::clone::Clone for MediaPixelFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPixelFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaPixelFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPixelFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPixelFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPixelFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaPixelFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct MediaPropertySet(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl MediaPropertySet {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaPropertySet, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Lookup)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasKey)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Insert)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for MediaPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for MediaPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for MediaPropertySet {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for MediaPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for MediaPropertySet {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.MediaPropertySet;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};g16;cinterface(IInspectable)))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for MediaPropertySet {
    type Vtable = super::super::Foundation::Collections::IMap_Vtbl<::windows::core::GUID, ::windows::core::IInspectable>;
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for MediaPropertySet {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaPropertySet";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for MediaPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &MediaPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPropertySet> for ::windows::core::IUnknown {
    fn from(value: MediaPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPropertySet> for ::windows::core::IUnknown {
    fn from(value: &MediaPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaPropertySet> for ::windows::core::IInspectable {
    fn from(value: MediaPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaPropertySet> for ::windows::core::IInspectable {
    fn from(value: &MediaPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<MediaPropertySet> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPropertySet) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&MediaPropertySet> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPropertySet) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>> for MediaPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>> for &MediaPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<MediaPropertySet> for super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPropertySet) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&MediaPropertySet> for super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPropertySet) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> for MediaPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> for &MediaPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for MediaPropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for MediaPropertySet {}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct MediaRatio(::windows::core::IUnknown);
impl MediaRatio {
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetNumerator(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNumerator)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Numerator(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Numerator)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetDenominator(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDenominator)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Denominator(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Denominator)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaRatio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaRatio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaRatio {}
impl ::core::fmt::Debug for MediaRatio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaRatio").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaRatio {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.MediaRatio;{d2d0fee5-8929-401d-ac78-7d357e378163})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaRatio {
    type Vtable = IMediaRatio_Vtbl;
    const IID: ::windows::core::GUID = <IMediaRatio as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaRatio {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaRatio";
}
impl ::core::convert::From<MediaRatio> for ::windows::core::IUnknown {
    fn from(value: MediaRatio) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaRatio> for ::windows::core::IUnknown {
    fn from(value: &MediaRatio) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaRatio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaRatio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaRatio> for ::windows::core::IInspectable {
    fn from(value: MediaRatio) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaRatio> for ::windows::core::IInspectable {
    fn from(value: &MediaRatio) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaRatio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaRatio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaRatio {}
unsafe impl ::core::marker::Sync for MediaRatio {}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaRotation(pub i32);
impl MediaRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaRotation {}
impl ::core::clone::Clone for MediaRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaRotation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaThumbnailFormat(pub i32);
impl MediaThumbnailFormat {
    pub const Bmp: Self = Self(0i32);
    pub const Bgra8: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaThumbnailFormat {}
impl ::core::clone::Clone for MediaThumbnailFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaThumbnailFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaThumbnailFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaThumbnailFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaThumbnailFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaThumbnailFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaThumbnailFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
pub struct Mpeg2ProfileIds {}
impl Mpeg2ProfileIds {
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Simple() -> ::windows::core::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Simple)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Main() -> ::windows::core::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Main)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SignalNoiseRatioScalable() -> ::windows::core::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalNoiseRatioScalable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SpatiallyScalable() -> ::windows::core::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SpatiallyScalable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn High() -> ::windows::core::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).High)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMpeg2ProfileIdsStatics<R, F: FnOnce(&IMpeg2ProfileIdsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Mpeg2ProfileIds, IMpeg2ProfileIdsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for Mpeg2ProfileIds {
    const NAME: &'static str = "Windows.Media.MediaProperties.Mpeg2ProfileIds";
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SphericalVideoFrameFormat(pub i32);
impl SphericalVideoFrameFormat {
    pub const None: Self = Self(0i32);
    pub const Unsupported: Self = Self(1i32);
    pub const Equirectangular: Self = Self(2i32);
}
impl ::core::marker::Copy for SphericalVideoFrameFormat {}
impl ::core::clone::Clone for SphericalVideoFrameFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SphericalVideoFrameFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SphericalVideoFrameFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for SphericalVideoFrameFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SphericalVideoFrameFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SphericalVideoFrameFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.SphericalVideoFrameFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StereoscopicVideoPackingMode(pub i32);
impl StereoscopicVideoPackingMode {
    pub const None: Self = Self(0i32);
    pub const SideBySide: Self = Self(1i32);
    pub const TopBottom: Self = Self(2i32);
}
impl ::core::marker::Copy for StereoscopicVideoPackingMode {}
impl ::core::clone::Clone for StereoscopicVideoPackingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StereoscopicVideoPackingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StereoscopicVideoPackingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for StereoscopicVideoPackingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StereoscopicVideoPackingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StereoscopicVideoPackingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.StereoscopicVideoPackingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct TimedMetadataEncodingProperties(::windows::core::IUnknown);
impl TimedMetadataEncodingProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedMetadataEncodingProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<MediaPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetSubtype<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubtype)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtype)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetFormatUserData(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITimedMetadataEncodingProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFormatUserData)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn GetFormatUserData(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITimedMetadataEncodingProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetFormatUserData)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Copy(&self) -> ::windows::core::Result<TimedMetadataEncodingProperties> {
        let this = &::windows::core::Interface::cast::<ITimedMetadataEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Copy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreatePgs() -> ::windows::core::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePgs)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateSrt() -> ::windows::core::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSrt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateSsa(formatuserdata: &[u8]) -> ::windows::core::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSsa)(::core::mem::transmute_copy(this), formatuserdata.len() as u32, ::core::mem::transmute(formatuserdata.as_ptr()), &mut result__).from_abi::<TimedMetadataEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateVobSub(formatuserdata: &[u8]) -> ::windows::core::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateVobSub)(::core::mem::transmute_copy(this), formatuserdata.len() as u32, ::core::mem::transmute(formatuserdata.as_ptr()), &mut result__).from_abi::<TimedMetadataEncodingProperties>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimedMetadataEncodingPropertiesStatics<R, F: FnOnce(&ITimedMetadataEncodingPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedMetadataEncodingProperties, ITimedMetadataEncodingPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TimedMetadataEncodingProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataEncodingProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataEncodingProperties {}
impl ::core::fmt::Debug for TimedMetadataEncodingProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataEncodingProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataEncodingProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.TimedMetadataEncodingProperties;{b4002af6-acd4-4e5a-a24b-5d7498a8b8c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TimedMetadataEncodingProperties {
    type Vtable = IMediaEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = <IMediaEncodingProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TimedMetadataEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.TimedMetadataEncodingProperties";
}
impl ::core::convert::From<TimedMetadataEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: TimedMetadataEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: &TimedMetadataEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedMetadataEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: TimedMetadataEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: &TimedMetadataEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TimedMetadataEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: TimedMetadataEncodingProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TimedMetadataEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &TimedMetadataEncodingProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaEncodingProperties> for TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaEncodingProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaEncodingProperties> for &TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaEncodingProperties> {
        ::core::convert::TryInto::<IMediaEncodingProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TimedMetadataEncodingProperties {}
unsafe impl ::core::marker::Sync for TimedMetadataEncodingProperties {}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct VideoEncodingProperties(::windows::core::IUnknown);
impl VideoEncodingProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoEncodingProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<MediaPropertySet> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetSubtype<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSubtype)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtype)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitrate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Bitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWidth)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn FrameRate(&self) -> ::windows::core::Result<MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaRatio>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn PixelAspectRatio(&self) -> ::windows::core::Result<MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PixelAspectRatio)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaRatio>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetFormatUserData(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFormatUserData)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn GetFormatUserData(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetFormatUserData)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SetProfileId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProfileId)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn ProfileId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProfileId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn StereoscopicVideoPackingMode(&self) -> ::windows::core::Result<StereoscopicVideoPackingMode> {
        let this = &::windows::core::Interface::cast::<IVideoEncodingProperties3>(self)?;
        unsafe {
            let mut result__: StereoscopicVideoPackingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StereoscopicVideoPackingMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StereoscopicVideoPackingMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn SphericalVideoFrameFormat(&self) -> ::windows::core::Result<SphericalVideoFrameFormat> {
        let this = &::windows::core::Interface::cast::<IVideoEncodingProperties4>(self)?;
        unsafe {
            let mut result__: SphericalVideoFrameFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SphericalVideoFrameFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SphericalVideoFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn Copy(&self) -> ::windows::core::Result<VideoEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IVideoEncodingProperties5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Copy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateH264() -> ::windows::core::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateH264)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateMpeg2() -> ::windows::core::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMpeg2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateUncompressed<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(subtype: Param0, width: u32, height: u32) -> ::windows::core::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateUncompressed)(::core::mem::transmute_copy(this), subtype.into_param().abi(), width, height, &mut result__).from_abi::<VideoEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    pub fn CreateHevc() -> ::windows::core::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateHevc)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoEncodingProperties>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVideoEncodingPropertiesStatics<R, F: FnOnce(&IVideoEncodingPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoEncodingProperties, IVideoEncodingPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IVideoEncodingPropertiesStatics2<R, F: FnOnce(&IVideoEncodingPropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoEncodingProperties, IVideoEncodingPropertiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VideoEncodingProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoEncodingProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoEncodingProperties {}
impl ::core::fmt::Debug for VideoEncodingProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoEncodingProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoEncodingProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.VideoEncodingProperties;{76ee6c9a-37c2-4f2a-880a-1282bbb4373d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VideoEncodingProperties {
    type Vtable = IVideoEncodingProperties_Vtbl;
    const IID: ::windows::core::GUID = <IVideoEncodingProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VideoEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.VideoEncodingProperties";
}
impl ::core::convert::From<VideoEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: VideoEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoEncodingProperties> for ::windows::core::IUnknown {
    fn from(value: &VideoEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: VideoEncodingProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoEncodingProperties> for ::windows::core::IInspectable {
    fn from(value: &VideoEncodingProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoEncodingProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoEncodingProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaEncodingProperties> for VideoEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaEncodingProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaEncodingProperties> for &VideoEncodingProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaEncodingProperties> {
        ::core::convert::TryInto::<IMediaEncodingProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoEncodingProperties {}
unsafe impl ::core::marker::Sync for VideoEncodingProperties {}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VideoEncodingQuality(pub i32);
impl VideoEncodingQuality {
    pub const Auto: Self = Self(0i32);
    pub const HD1080p: Self = Self(1i32);
    pub const HD720p: Self = Self(2i32);
    pub const Wvga: Self = Self(3i32);
    pub const Ntsc: Self = Self(4i32);
    pub const Pal: Self = Self(5i32);
    pub const Vga: Self = Self(6i32);
    pub const Qvga: Self = Self(7i32);
    pub const Uhd2160p: Self = Self(8i32);
    pub const Uhd4320p: Self = Self(9i32);
}
impl ::core::marker::Copy for VideoEncodingQuality {}
impl ::core::clone::Clone for VideoEncodingQuality {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoEncodingQuality {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VideoEncodingQuality {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoEncodingQuality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoEncodingQuality").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoEncodingQuality {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.VideoEncodingQuality;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
