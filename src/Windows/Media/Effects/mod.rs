#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AudioCaptureEffectsManager(::windows::runtime::IInspectable);
impl AudioCaptureEffectsManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Effects`, `Foundation`*"]
    pub fn AudioCaptureEffectsChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Effects`, `Foundation`*"]
    pub fn RemoveAudioCaptureEffectsChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn GetAudioCaptureEffects(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioEffect>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioCaptureEffectsManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioCaptureEffectsManager;{8f85c271-038d-4393-8298-540110608eef})");
}
unsafe impl ::windows::runtime::Interface for AudioCaptureEffectsManager {
    type Vtable = IAudioCaptureEffectsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2407907953, 909, 17299, [130, 152, 84, 1, 16, 96, 142, 239]);
}
impl ::windows::runtime::RuntimeName for AudioCaptureEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioCaptureEffectsManager";
}
impl ::std::convert::From<AudioCaptureEffectsManager> for ::windows::runtime::IUnknown {
    fn from(value: AudioCaptureEffectsManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioCaptureEffectsManager> for ::windows::runtime::IUnknown {
    fn from(value: &AudioCaptureEffectsManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AudioCaptureEffectsManager> for ::windows::runtime::IInspectable {
    fn from(value: AudioCaptureEffectsManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioCaptureEffectsManager> for ::windows::runtime::IInspectable {
    fn from(value: &AudioCaptureEffectsManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioCaptureEffectsManager {}
unsafe impl ::std::marker::Sync for AudioCaptureEffectsManager {}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AudioEffect(::windows::runtime::IInspectable);
impl AudioEffect {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn AudioEffectType(&self) -> ::windows::runtime::Result<AudioEffectType> {
        let this = self;
        unsafe {
            let mut result__: AudioEffectType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AudioEffectType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioEffect;{34aafa51-9207-4055-be93-6e5734a86ae4})");
}
unsafe impl ::windows::runtime::Interface for AudioEffect {
    type Vtable = IAudioEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(883620433, 37383, 16469, [190, 147, 110, 87, 52, 168, 106, 228]);
}
impl ::windows::runtime::RuntimeName for AudioEffect {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffect";
}
impl ::std::convert::From<AudioEffect> for ::windows::runtime::IUnknown {
    fn from(value: AudioEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioEffect> for ::windows::runtime::IUnknown {
    fn from(value: &AudioEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AudioEffect> for ::windows::runtime::IInspectable {
    fn from(value: AudioEffect) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioEffect> for ::windows::runtime::IInspectable {
    fn from(value: &AudioEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AudioEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioEffect {}
unsafe impl ::std::marker::Sync for AudioEffect {}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AudioEffectDefinition(::windows::runtime::IInspectable);
impl AudioEffectDefinition {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(activatableclassid: Param0) -> ::windows::runtime::Result<AudioEffectDefinition> {
        Self::IAudioEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi(), &mut result__).from_abi::<AudioEffectDefinition>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn CreateWithProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(activatableclassid: Param0, props: Param1) -> ::windows::runtime::Result<AudioEffectDefinition> {
        Self::IAudioEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi(), props.into_param().abi(), &mut result__).from_abi::<AudioEffectDefinition>(result__)
        })
    }
    pub fn IAudioEffectDefinitionFactory<R, F: FnOnce(&IAudioEffectDefinitionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AudioEffectDefinition, IAudioEffectDefinitionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioEffectDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioEffectDefinition;{e4d7f974-7d80-4f73-9089-e31c9db9c294})");
}
unsafe impl ::windows::runtime::Interface for AudioEffectDefinition {
    type Vtable = IAudioEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3839359348, 32128, 20339, [144, 137, 227, 28, 157, 185, 194, 148]);
}
impl ::windows::runtime::RuntimeName for AudioEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffectDefinition";
}
impl ::std::convert::From<AudioEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: AudioEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &AudioEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AudioEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: AudioEffectDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &AudioEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AudioEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AudioEffectDefinition> for IAudioEffectDefinition {
    fn from(value: AudioEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioEffectDefinition> for IAudioEffectDefinition {
    fn from(value: &AudioEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioEffectDefinition> for AudioEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioEffectDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IAudioEffectDefinition>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioEffectDefinition> for &AudioEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioEffectDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IAudioEffectDefinition>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for AudioEffectDefinition {}
unsafe impl ::std::marker::Sync for AudioEffectDefinition {}
#[doc = "*Required features: `Media_Effects`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioEffectType(pub i32);
impl AudioEffectType {
    pub const Other: AudioEffectType = AudioEffectType(0i32);
    pub const AcousticEchoCancellation: AudioEffectType = AudioEffectType(1i32);
    pub const NoiseSuppression: AudioEffectType = AudioEffectType(2i32);
    pub const AutomaticGainControl: AudioEffectType = AudioEffectType(3i32);
    pub const BeamForming: AudioEffectType = AudioEffectType(4i32);
    pub const ConstantToneRemoval: AudioEffectType = AudioEffectType(5i32);
    pub const Equalizer: AudioEffectType = AudioEffectType(6i32);
    pub const LoudnessEqualizer: AudioEffectType = AudioEffectType(7i32);
    pub const BassBoost: AudioEffectType = AudioEffectType(8i32);
    pub const VirtualSurround: AudioEffectType = AudioEffectType(9i32);
    pub const VirtualHeadphones: AudioEffectType = AudioEffectType(10i32);
    pub const SpeakerFill: AudioEffectType = AudioEffectType(11i32);
    pub const RoomCorrection: AudioEffectType = AudioEffectType(12i32);
    pub const BassManagement: AudioEffectType = AudioEffectType(13i32);
    pub const EnvironmentalEffects: AudioEffectType = AudioEffectType(14i32);
    pub const SpeakerProtection: AudioEffectType = AudioEffectType(15i32);
    pub const SpeakerCompensation: AudioEffectType = AudioEffectType(16i32);
    pub const DynamicRangeCompression: AudioEffectType = AudioEffectType(17i32);
    pub const FarFieldBeamForming: AudioEffectType = AudioEffectType(18i32);
    pub const DeepNoiseSuppression: AudioEffectType = AudioEffectType(19i32);
}
impl ::std::convert::From<i32> for AudioEffectType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioEffectType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioEffectType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.AudioEffectType;i4)");
}
impl ::windows::runtime::DefaultType for AudioEffectType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Effects`*"]
pub struct AudioEffectsManager {}
impl AudioEffectsManager {
    #[cfg(feature = "Media_Render")]
    #[doc = "*Required features: `Media_Effects`, `Media_Render`*"]
    pub fn CreateAudioRenderEffectsManager<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0, category: super::Render::AudioRenderCategory) -> ::windows::runtime::Result<AudioRenderEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), category, &mut result__).from_abi::<AudioRenderEffectsManager>(result__)
        })
    }
    #[cfg(feature = "Media_Render")]
    #[doc = "*Required features: `Media_Effects`, `Media_Render`*"]
    pub fn CreateAudioRenderEffectsManagerWithMode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing) -> ::windows::runtime::Result<AudioRenderEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), category, mode, &mut result__).from_abi::<AudioRenderEffectsManager>(result__)
        })
    }
    #[cfg(feature = "Media_Capture")]
    #[doc = "*Required features: `Media_Effects`, `Media_Capture`*"]
    pub fn CreateAudioCaptureEffectsManager<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0, category: super::Capture::MediaCategory) -> ::windows::runtime::Result<AudioCaptureEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), category, &mut result__).from_abi::<AudioCaptureEffectsManager>(result__)
        })
    }
    #[cfg(feature = "Media_Capture")]
    #[doc = "*Required features: `Media_Effects`, `Media_Capture`*"]
    pub fn CreateAudioCaptureEffectsManagerWithMode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0, category: super::Capture::MediaCategory, mode: super::AudioProcessing) -> ::windows::runtime::Result<AudioCaptureEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), category, mode, &mut result__).from_abi::<AudioCaptureEffectsManager>(result__)
        })
    }
    pub fn IAudioEffectsManagerStatics<R, F: FnOnce(&IAudioEffectsManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AudioEffectsManager, IAudioEffectsManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AudioEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffectsManager";
}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AudioRenderEffectsManager(::windows::runtime::IInspectable);
impl AudioRenderEffectsManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Effects`, `Foundation`*"]
    pub fn AudioRenderEffectsChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Effects`, `Foundation`*"]
    pub fn RemoveAudioRenderEffectsChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn GetAudioRenderEffects(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioEffect>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Effects`, `Storage_Streams`*"]
    pub fn EffectsProviderThumbnail(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = &::windows::runtime::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn EffectsProviderSettingsLabel(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ShowSettingsUI(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioRenderEffectsManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioRenderEffectsManager;{4dc98966-8751-42b2-bfcb-39ca7864bd47})");
}
unsafe impl ::windows::runtime::Interface for AudioRenderEffectsManager {
    type Vtable = IAudioRenderEffectsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1305053542, 34641, 17074, [191, 203, 57, 202, 120, 100, 189, 71]);
}
impl ::windows::runtime::RuntimeName for AudioRenderEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioRenderEffectsManager";
}
impl ::std::convert::From<AudioRenderEffectsManager> for ::windows::runtime::IUnknown {
    fn from(value: AudioRenderEffectsManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioRenderEffectsManager> for ::windows::runtime::IUnknown {
    fn from(value: &AudioRenderEffectsManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioRenderEffectsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioRenderEffectsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AudioRenderEffectsManager> for ::windows::runtime::IInspectable {
    fn from(value: AudioRenderEffectsManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioRenderEffectsManager> for ::windows::runtime::IInspectable {
    fn from(value: &AudioRenderEffectsManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioRenderEffectsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AudioRenderEffectsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioRenderEffectsManager {}
unsafe impl ::std::marker::Sync for AudioRenderEffectsManager {}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CompositeVideoFrameContext(::windows::runtime::IInspectable);
impl CompositeVideoFrameContext {
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`, `Graphics_DirectX_Direct3D11`*"]
    pub fn SurfacesToOverlay(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn BackgroundFrame(&self) -> ::windows::runtime::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn OutputFrame(&self) -> ::windows::runtime::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))]
    #[doc = "*Required features: `Media_Effects`, `Graphics_DirectX_Direct3D11`, `Media_Editing`*"]
    pub fn GetOverlayForSurface<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>(&self, surfacetooverlay: Param0) -> ::windows::runtime::Result<super::Editing::MediaOverlay> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), surfacetooverlay.into_param().abi(), &mut result__).from_abi::<super::Editing::MediaOverlay>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositeVideoFrameContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.CompositeVideoFrameContext;{6c30024b-f514-4278-a5f7-b9188049d110})");
}
unsafe impl ::windows::runtime::Interface for CompositeVideoFrameContext {
    type Vtable = ICompositeVideoFrameContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1815085643, 62740, 17016, [165, 247, 185, 24, 128, 73, 209, 16]);
}
impl ::windows::runtime::RuntimeName for CompositeVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.CompositeVideoFrameContext";
}
impl ::std::convert::From<CompositeVideoFrameContext> for ::windows::runtime::IUnknown {
    fn from(value: CompositeVideoFrameContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CompositeVideoFrameContext> for ::windows::runtime::IUnknown {
    fn from(value: &CompositeVideoFrameContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CompositeVideoFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CompositeVideoFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CompositeVideoFrameContext> for ::windows::runtime::IInspectable {
    fn from(value: CompositeVideoFrameContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CompositeVideoFrameContext> for ::windows::runtime::IInspectable {
    fn from(value: &CompositeVideoFrameContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CompositeVideoFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CompositeVideoFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CompositeVideoFrameContext {}
unsafe impl ::std::marker::Sync for CompositeVideoFrameContext {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioCaptureEffectsManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioCaptureEffectsManager {
    type Vtable = IAudioCaptureEffectsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2407907953, 909, 17299, [130, 152, 84, 1, 16, 96, 142, 239]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioCaptureEffectsManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioEffect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioEffect {
    type Vtable = IAudioEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(883620433, 37383, 16469, [190, 147, 110, 87, 52, 168, 106, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AudioEffectType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Media_Effects`*"]
pub struct IAudioEffectDefinition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioEffectDefinition {
    type Vtable = IAudioEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3839359348, 32128, 20339, [144, 137, 227, 28, 157, 185, 194, 148]);
}
impl IAudioEffectDefinition {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAudioEffectDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e4d7f974-7d80-4f73-9089-e31c9db9c294}");
}
impl ::std::convert::From<IAudioEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAudioEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAudioEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IAudioEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: IAudioEffectDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &IAudioEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAudioEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IAudioEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioEffectDefinitionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioEffectDefinitionFactory {
    type Vtable = IAudioEffectDefinitionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2384307782, 59141, 17901, [138, 43, 252, 78, 79, 64, 90, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectDefinitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, props: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioEffectsManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioEffectsManagerStatics {
    type Vtable = IAudioEffectsManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1715497988, 34554, 18380, [163, 21, 244, 137, 216, 195, 254, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Render")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, category: super::Render::AudioRenderCategory, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    #[cfg(feature = "Media_Render")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    #[cfg(feature = "Media_Capture")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, category: super::Capture::MediaCategory, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
    #[cfg(feature = "Media_Capture")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, category: super::Capture::MediaCategory, mode: super::AudioProcessing, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioRenderEffectsManager {
    type Vtable = IAudioRenderEffectsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1305053542, 34641, 17074, [191, 203, 57, 202, 120, 100, 189, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioRenderEffectsManager2 {
    type Vtable = IAudioRenderEffectsManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2823081225, 24268, 17587, [187, 78, 29, 176, 114, 135, 19, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Media_Effects`*"]
pub struct IBasicAudioEffect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBasicAudioEffect {
    type Vtable = IBasicAudioEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2349214803, 27584, 18616, [169, 154, 75, 65, 85, 15, 19, 89]);
}
impl IBasicAudioEffect {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn UseInputFrameForOutput(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`, `Media_MediaProperties`*"]
    pub fn SupportedEncodingProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Effects`, `Media_MediaProperties`*"]
    pub fn SetEncodingProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), encodingproperties.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ProcessFrame<'a, Param0: ::windows::runtime::IntoParam<'a, ProcessAudioFrameContext>>(&self, context: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), context.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), reason).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn DiscardQueuedFrames(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn SetProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBasicAudioEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{8c062c53-6bc0-48b8-a99a-4b41550f1359}");
}
impl ::std::convert::From<IBasicAudioEffect> for ::windows::runtime::IUnknown {
    fn from(value: IBasicAudioEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBasicAudioEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IBasicAudioEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBasicAudioEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBasicAudioEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBasicAudioEffect> for ::windows::runtime::IInspectable {
    fn from(value: IBasicAudioEffect) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBasicAudioEffect> for ::windows::runtime::IInspectable {
    fn from(value: &IBasicAudioEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBasicAudioEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBasicAudioEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IBasicAudioEffect> for super::IMediaExtension {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBasicAudioEffect) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IBasicAudioEffect> for super::IMediaExtension {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBasicAudioEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaExtension> for IBasicAudioEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaExtension> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaExtension> for &IBasicAudioEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaExtension> {
        ::std::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicAudioEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingproperties: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: MediaEffectClosedReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Media_Effects`*"]
pub struct IBasicVideoEffect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBasicVideoEffect {
    type Vtable = IBasicVideoEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2187511791, 45920, 16574, [148, 155, 47, 244, 47, 243, 86, 147]);
}
impl IBasicVideoEffect {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn SupportedMemoryTypes(&self) -> ::windows::runtime::Result<MediaMemoryTypes> {
        let this = self;
        unsafe {
            let mut result__: MediaMemoryTypes = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaMemoryTypes>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn TimeIndependent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`, `Media_MediaProperties`*"]
    pub fn SupportedEncodingProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Effects`, `Graphics_DirectX_Direct3D11`, `Media_MediaProperties`*"]
    pub fn SetEncodingProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::VideoEncodingProperties>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>>(&self, encodingproperties: Param0, device: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), encodingproperties.into_param().abi(), device.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ProcessFrame<'a, Param0: ::windows::runtime::IntoParam<'a, ProcessVideoFrameContext>>(&self, context: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), context.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), reason).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn DiscardQueuedFrames(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn SetProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBasicVideoEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{8262c7ef-b360-40be-949b-2ff42ff35693}");
}
impl ::std::convert::From<IBasicVideoEffect> for ::windows::runtime::IUnknown {
    fn from(value: IBasicVideoEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBasicVideoEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IBasicVideoEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBasicVideoEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBasicVideoEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBasicVideoEffect> for ::windows::runtime::IInspectable {
    fn from(value: IBasicVideoEffect) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBasicVideoEffect> for ::windows::runtime::IInspectable {
    fn from(value: &IBasicVideoEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBasicVideoEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBasicVideoEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IBasicVideoEffect> for super::IMediaExtension {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBasicVideoEffect) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IBasicVideoEffect> for super::IMediaExtension {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBasicVideoEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaExtension> for IBasicVideoEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaExtension> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaExtension> for &IBasicVideoEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaExtension> {
        ::std::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicVideoEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaMemoryTypes) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingproperties: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: MediaEffectClosedReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositeVideoFrameContext(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositeVideoFrameContext {
    type Vtable = ICompositeVideoFrameContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1815085643, 62740, 17016, [165, 247, 185, 24, 128, 73, 209, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeVideoFrameContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, surfacetooverlay: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessAudioFrameContext(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessAudioFrameContext {
    type Vtable = IProcessAudioFrameContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1289300294, 4642, 18983, [165, 134, 251, 62, 32, 39, 50, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessAudioFrameContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessVideoFrameContext(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessVideoFrameContext {
    type Vtable = IProcessVideoFrameContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(661589547, 25697, 16414, [186, 120, 15, 218, 214, 17, 78, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessVideoFrameContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISlowMotionEffectDefinition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISlowMotionEffectDefinition {
    type Vtable = ISlowMotionEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(889535696, 5996, 18275, [130, 196, 27, 2, 219, 227, 23, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlowMotionEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Media_Effects`*"]
pub struct IVideoCompositor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoCompositor {
    type Vtable = IVideoCompositor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2232464446, 16908, 16911, [150, 199, 124, 152, 187, 161, 252, 85]);
}
impl IVideoCompositor {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn TimeIndependent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Effects`, `Graphics_DirectX_Direct3D11`, `Media_MediaProperties`*"]
    pub fn SetEncodingProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::VideoEncodingProperties>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>>(&self, backgroundproperties: Param0, device: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), backgroundproperties.into_param().abi(), device.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn CompositeFrame<'a, Param0: ::windows::runtime::IntoParam<'a, CompositeVideoFrameContext>>(&self, context: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), context.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), reason).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn DiscardQueuedFrames(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn SetProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IVideoCompositor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{8510b43e-420c-420f-96c7-7c98bba1fc55}");
}
impl ::std::convert::From<IVideoCompositor> for ::windows::runtime::IUnknown {
    fn from(value: IVideoCompositor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVideoCompositor> for ::windows::runtime::IUnknown {
    fn from(value: &IVideoCompositor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVideoCompositor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVideoCompositor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IVideoCompositor> for ::windows::runtime::IInspectable {
    fn from(value: IVideoCompositor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IVideoCompositor> for ::windows::runtime::IInspectable {
    fn from(value: &IVideoCompositor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IVideoCompositor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IVideoCompositor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IVideoCompositor> for super::IMediaExtension {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IVideoCompositor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IVideoCompositor> for super::IMediaExtension {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IVideoCompositor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaExtension> for IVideoCompositor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaExtension> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaExtension> for &IVideoCompositor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaExtension> {
        ::std::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, backgroundproperties: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: MediaEffectClosedReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Media_Effects`*"]
pub struct IVideoCompositorDefinition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoCompositorDefinition {
    type Vtable = IVideoCompositorDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2034677968, 8208, 19171, [154, 178, 44, 239, 66, 237, 212, 210]);
}
impl IVideoCompositorDefinition {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IVideoCompositorDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{7946b8d0-2010-4ae3-9ab2-2cef42edd4d2}");
}
impl ::std::convert::From<IVideoCompositorDefinition> for ::windows::runtime::IUnknown {
    fn from(value: IVideoCompositorDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVideoCompositorDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &IVideoCompositorDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVideoCompositorDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVideoCompositorDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IVideoCompositorDefinition> for ::windows::runtime::IInspectable {
    fn from(value: IVideoCompositorDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IVideoCompositorDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &IVideoCompositorDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IVideoCompositorDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IVideoCompositorDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositorDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoCompositorDefinitionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoCompositorDefinitionFactory {
    type Vtable = IVideoCompositorDefinitionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1130822928, 26808, 19794, [137, 182, 2, 169, 104, 204, 168, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositorDefinitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, props: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Media_Effects`*"]
pub struct IVideoEffectDefinition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoEffectDefinition {
    type Vtable = IVideoEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(972262640, 36111, 20286, [132, 252, 45, 70, 165, 41, 121, 67]);
}
impl IVideoEffectDefinition {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IVideoEffectDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{39f38cf0-8d0f-4f3e-84fc-2d46a5297943}");
}
impl ::std::convert::From<IVideoEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: IVideoEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVideoEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &IVideoEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVideoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVideoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IVideoEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: IVideoEffectDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IVideoEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &IVideoEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IVideoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IVideoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoEffectDefinitionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoEffectDefinitionFactory {
    type Vtable = IVideoEffectDefinitionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2168691534, 28211, 17039, [157, 33, 181, 170, 254, 247, 97, 124]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectDefinitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, props: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoTransformEffectDefinition {
    type Vtable = IVideoTransformEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2523183978, 7846, 19110, [128, 116, 171, 232, 133, 30, 202, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::MediaProperties::MediaRotation) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::MediaProperties::MediaRotation) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::MediaProperties::MediaMirroringOptions) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::MediaProperties::MediaMirroringOptions) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_Transcoding")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))] usize,
    #[cfg(feature = "Media_Transcoding")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoTransformEffectDefinition2 {
    type Vtable = IVideoTransformEffectDefinition2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4037544095, 26312, 18068, [159, 217, 17, 54, 171, 247, 68, 74]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoTransformSphericalProjection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoTransformSphericalProjection {
    type Vtable = IVideoTransformSphericalProjection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3477340656, 39922, 19513, [159, 65, 224, 34, 81, 74, 132, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformSphericalProjection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_Playback")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Playback::SphericalVideoProjectionMode) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Playback"))] usize,
    #[cfg(feature = "Media_Playback")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Playback::SphericalVideoProjectionMode) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Playback"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[doc = "*Required features: `Media_Effects`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaEffectClosedReason(pub i32);
impl MediaEffectClosedReason {
    pub const Done: MediaEffectClosedReason = MediaEffectClosedReason(0i32);
    pub const UnknownError: MediaEffectClosedReason = MediaEffectClosedReason(1i32);
    pub const UnsupportedEncodingFormat: MediaEffectClosedReason = MediaEffectClosedReason(2i32);
    pub const EffectCurrentlyUnloaded: MediaEffectClosedReason = MediaEffectClosedReason(3i32);
}
impl ::std::convert::From<i32> for MediaEffectClosedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaEffectClosedReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaEffectClosedReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.MediaEffectClosedReason;i4)");
}
impl ::windows::runtime::DefaultType for MediaEffectClosedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Effects`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaMemoryTypes(pub i32);
impl MediaMemoryTypes {
    pub const Gpu: MediaMemoryTypes = MediaMemoryTypes(0i32);
    pub const Cpu: MediaMemoryTypes = MediaMemoryTypes(1i32);
    pub const GpuAndCpu: MediaMemoryTypes = MediaMemoryTypes(2i32);
}
impl ::std::convert::From<i32> for MediaMemoryTypes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaMemoryTypes {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaMemoryTypes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.MediaMemoryTypes;i4)");
}
impl ::windows::runtime::DefaultType for MediaMemoryTypes {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProcessAudioFrameContext(::windows::runtime::IInspectable);
impl ProcessAudioFrameContext {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn InputFrame(&self) -> ::windows::runtime::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn OutputFrame(&self) -> ::windows::runtime::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioFrame>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessAudioFrameContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.ProcessAudioFrameContext;{4cd92946-1222-4a27-a586-fb3e20273255})");
}
unsafe impl ::windows::runtime::Interface for ProcessAudioFrameContext {
    type Vtable = IProcessAudioFrameContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1289300294, 4642, 18983, [165, 134, 251, 62, 32, 39, 50, 85]);
}
impl ::windows::runtime::RuntimeName for ProcessAudioFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ProcessAudioFrameContext";
}
impl ::std::convert::From<ProcessAudioFrameContext> for ::windows::runtime::IUnknown {
    fn from(value: ProcessAudioFrameContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProcessAudioFrameContext> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessAudioFrameContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessAudioFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProcessAudioFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProcessAudioFrameContext> for ::windows::runtime::IInspectable {
    fn from(value: ProcessAudioFrameContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProcessAudioFrameContext> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessAudioFrameContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessAudioFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessAudioFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProcessAudioFrameContext {}
unsafe impl ::std::marker::Sync for ProcessAudioFrameContext {}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProcessVideoFrameContext(::windows::runtime::IInspectable);
impl ProcessVideoFrameContext {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn InputFrame(&self) -> ::windows::runtime::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn OutputFrame(&self) -> ::windows::runtime::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessVideoFrameContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.ProcessVideoFrameContext;{276f0e2b-6461-401e-ba78-0fdad6114eec})");
}
unsafe impl ::windows::runtime::Interface for ProcessVideoFrameContext {
    type Vtable = IProcessVideoFrameContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(661589547, 25697, 16414, [186, 120, 15, 218, 214, 17, 78, 236]);
}
impl ::windows::runtime::RuntimeName for ProcessVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ProcessVideoFrameContext";
}
impl ::std::convert::From<ProcessVideoFrameContext> for ::windows::runtime::IUnknown {
    fn from(value: ProcessVideoFrameContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProcessVideoFrameContext> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessVideoFrameContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessVideoFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProcessVideoFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProcessVideoFrameContext> for ::windows::runtime::IInspectable {
    fn from(value: ProcessVideoFrameContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProcessVideoFrameContext> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessVideoFrameContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessVideoFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessVideoFrameContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProcessVideoFrameContext {}
unsafe impl ::std::marker::Sync for ProcessVideoFrameContext {}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SlowMotionEffectDefinition(::windows::runtime::IInspectable);
impl SlowMotionEffectDefinition {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SlowMotionEffectDefinition, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn TimeStretchRate(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn SetTimeStretchRate(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IVideoEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::runtime::Interface::cast::<IVideoEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SlowMotionEffectDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.SlowMotionEffectDefinition;{35053cd0-176c-4763-82c4-1b02dbe31737})");
}
unsafe impl ::windows::runtime::Interface for SlowMotionEffectDefinition {
    type Vtable = ISlowMotionEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(889535696, 5996, 18275, [130, 196, 27, 2, 219, 227, 23, 55]);
}
impl ::windows::runtime::RuntimeName for SlowMotionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.SlowMotionEffectDefinition";
}
impl ::std::convert::From<SlowMotionEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: SlowMotionEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SlowMotionEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &SlowMotionEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SlowMotionEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: SlowMotionEffectDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SlowMotionEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &SlowMotionEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SlowMotionEffectDefinition> for IVideoEffectDefinition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SlowMotionEffectDefinition) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SlowMotionEffectDefinition> for IVideoEffectDefinition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SlowMotionEffectDefinition) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVideoEffectDefinition> for SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVideoEffectDefinition> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVideoEffectDefinition> for &SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVideoEffectDefinition> {
        ::std::convert::TryInto::<IVideoEffectDefinition>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SlowMotionEffectDefinition {}
unsafe impl ::std::marker::Sync for SlowMotionEffectDefinition {}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VideoCompositorDefinition(::windows::runtime::IInspectable);
impl VideoCompositorDefinition {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(activatableclassid: Param0) -> ::windows::runtime::Result<VideoCompositorDefinition> {
        Self::IVideoCompositorDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi(), &mut result__).from_abi::<VideoCompositorDefinition>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn CreateWithProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(activatableclassid: Param0, props: Param1) -> ::windows::runtime::Result<VideoCompositorDefinition> {
        Self::IVideoCompositorDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi(), props.into_param().abi(), &mut result__).from_abi::<VideoCompositorDefinition>(result__)
        })
    }
    pub fn IVideoCompositorDefinitionFactory<R, F: FnOnce(&IVideoCompositorDefinitionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VideoCompositorDefinition, IVideoCompositorDefinitionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoCompositorDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoCompositorDefinition;{7946b8d0-2010-4ae3-9ab2-2cef42edd4d2})");
}
unsafe impl ::windows::runtime::Interface for VideoCompositorDefinition {
    type Vtable = IVideoCompositorDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2034677968, 8208, 19171, [154, 178, 44, 239, 66, 237, 212, 210]);
}
impl ::windows::runtime::RuntimeName for VideoCompositorDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoCompositorDefinition";
}
impl ::std::convert::From<VideoCompositorDefinition> for ::windows::runtime::IUnknown {
    fn from(value: VideoCompositorDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VideoCompositorDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &VideoCompositorDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VideoCompositorDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VideoCompositorDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VideoCompositorDefinition> for ::windows::runtime::IInspectable {
    fn from(value: VideoCompositorDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VideoCompositorDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &VideoCompositorDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VideoCompositorDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VideoCompositorDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<VideoCompositorDefinition> for IVideoCompositorDefinition {
    fn from(value: VideoCompositorDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VideoCompositorDefinition> for IVideoCompositorDefinition {
    fn from(value: &VideoCompositorDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVideoCompositorDefinition> for VideoCompositorDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVideoCompositorDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVideoCompositorDefinition>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVideoCompositorDefinition> for &VideoCompositorDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVideoCompositorDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVideoCompositorDefinition>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for VideoCompositorDefinition {}
unsafe impl ::std::marker::Sync for VideoCompositorDefinition {}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VideoEffectDefinition(::windows::runtime::IInspectable);
impl VideoEffectDefinition {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(activatableclassid: Param0) -> ::windows::runtime::Result<VideoEffectDefinition> {
        Self::IVideoEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi(), &mut result__).from_abi::<VideoEffectDefinition>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn CreateWithProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(activatableclassid: Param0, props: Param1) -> ::windows::runtime::Result<VideoEffectDefinition> {
        Self::IVideoEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi(), props.into_param().abi(), &mut result__).from_abi::<VideoEffectDefinition>(result__)
        })
    }
    pub fn IVideoEffectDefinitionFactory<R, F: FnOnce(&IVideoEffectDefinitionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VideoEffectDefinition, IVideoEffectDefinitionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoEffectDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
unsafe impl ::windows::runtime::Interface for VideoEffectDefinition {
    type Vtable = IVideoEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(972262640, 36111, 20286, [132, 252, 45, 70, 165, 41, 121, 67]);
}
impl ::windows::runtime::RuntimeName for VideoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoEffectDefinition";
}
impl ::std::convert::From<VideoEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: VideoEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VideoEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &VideoEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VideoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VideoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VideoEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: VideoEffectDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VideoEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &VideoEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VideoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VideoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<VideoEffectDefinition> for IVideoEffectDefinition {
    fn from(value: VideoEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VideoEffectDefinition> for IVideoEffectDefinition {
    fn from(value: &VideoEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVideoEffectDefinition> for VideoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVideoEffectDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVideoEffectDefinition>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVideoEffectDefinition> for &VideoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVideoEffectDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVideoEffectDefinition>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for VideoEffectDefinition {}
unsafe impl ::std::marker::Sync for VideoEffectDefinition {}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VideoTransformEffectDefinition(::windows::runtime::IInspectable);
impl VideoTransformEffectDefinition {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VideoTransformEffectDefinition, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Media_Effects`, `UI`*"]
    pub fn PaddingColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Media_Effects`, `UI`*"]
    pub fn SetPaddingColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Effects`, `Foundation`*"]
    pub fn OutputSize(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Effects`, `Foundation`*"]
    pub fn SetOutputSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Effects`, `Foundation`*"]
    pub fn CropRectangle(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Effects`, `Foundation`*"]
    pub fn SetCropRectangle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Effects`, `Media_MediaProperties`*"]
    pub fn Rotation(&self) -> ::windows::runtime::Result<super::MediaProperties::MediaRotation> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::MediaProperties::MediaRotation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaRotation>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Effects`, `Media_MediaProperties`*"]
    pub fn SetRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Effects`, `Media_MediaProperties`*"]
    pub fn Mirror(&self) -> ::windows::runtime::Result<super::MediaProperties::MediaMirroringOptions> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::MediaProperties::MediaMirroringOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaMirroringOptions>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Effects`, `Media_MediaProperties`*"]
    pub fn SetMirror(&self, value: super::MediaProperties::MediaMirroringOptions) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Transcoding")]
    #[doc = "*Required features: `Media_Effects`, `Media_Transcoding`*"]
    pub fn SetProcessingAlgorithm(&self, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Transcoding")]
    #[doc = "*Required features: `Media_Effects`, `Media_Transcoding`*"]
    pub fn ProcessingAlgorithm(&self) -> ::windows::runtime::Result<super::Transcoding::MediaVideoProcessingAlgorithm> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::Transcoding::MediaVideoProcessingAlgorithm = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Transcoding::MediaVideoProcessingAlgorithm>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn SphericalProjection(&self) -> ::windows::runtime::Result<VideoTransformSphericalProjection> {
        let this = &::windows::runtime::Interface::cast::<IVideoTransformEffectDefinition2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VideoTransformSphericalProjection>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoTransformEffectDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoTransformEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
unsafe impl ::windows::runtime::Interface for VideoTransformEffectDefinition {
    type Vtable = IVideoEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(972262640, 36111, 20286, [132, 252, 45, 70, 165, 41, 121, 67]);
}
impl ::windows::runtime::RuntimeName for VideoTransformEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoTransformEffectDefinition";
}
impl ::std::convert::From<VideoTransformEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: VideoTransformEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VideoTransformEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &VideoTransformEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VideoTransformEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: VideoTransformEffectDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VideoTransformEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &VideoTransformEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<VideoTransformEffectDefinition> for IVideoEffectDefinition {
    fn from(value: VideoTransformEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VideoTransformEffectDefinition> for IVideoEffectDefinition {
    fn from(value: &VideoTransformEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVideoEffectDefinition> for VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVideoEffectDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVideoEffectDefinition>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVideoEffectDefinition> for &VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVideoEffectDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IVideoEffectDefinition>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for VideoTransformEffectDefinition {}
unsafe impl ::std::marker::Sync for VideoTransformEffectDefinition {}
#[doc = "*Required features: `Media_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VideoTransformSphericalProjection(::windows::runtime::IInspectable);
impl VideoTransformSphericalProjection {
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Effects`, `Media_MediaProperties`*"]
    pub fn FrameFormat(&self) -> ::windows::runtime::Result<super::MediaProperties::SphericalVideoFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: super::MediaProperties::SphericalVideoFrameFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::SphericalVideoFrameFormat>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Effects`, `Media_MediaProperties`*"]
    pub fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Playback")]
    #[doc = "*Required features: `Media_Effects`, `Media_Playback`*"]
    pub fn ProjectionMode(&self) -> ::windows::runtime::Result<super::Playback::SphericalVideoProjectionMode> {
        let this = self;
        unsafe {
            let mut result__: super::Playback::SphericalVideoProjectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Playback::SphericalVideoProjectionMode>(result__)
        }
    }
    #[cfg(feature = "Media_Playback")]
    #[doc = "*Required features: `Media_Effects`, `Media_Playback`*"]
    pub fn SetProjectionMode(&self, value: super::Playback::SphericalVideoProjectionMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn HorizontalFieldOfViewInDegrees(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Media_Effects`*"]
    pub fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Numerics`*"]
    pub fn ViewOrientation(&self) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Media_Effects`, `Foundation_Numerics`*"]
    pub fn SetViewOrientation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoTransformSphericalProjection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoTransformSphericalProjection;{cf4401f0-9bf2-4c39-9f41-e022514a8468})");
}
unsafe impl ::windows::runtime::Interface for VideoTransformSphericalProjection {
    type Vtable = IVideoTransformSphericalProjection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3477340656, 39922, 19513, [159, 65, 224, 34, 81, 74, 132, 104]);
}
impl ::windows::runtime::RuntimeName for VideoTransformSphericalProjection {
    const NAME: &'static str = "Windows.Media.Effects.VideoTransformSphericalProjection";
}
impl ::std::convert::From<VideoTransformSphericalProjection> for ::windows::runtime::IUnknown {
    fn from(value: VideoTransformSphericalProjection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VideoTransformSphericalProjection> for ::windows::runtime::IUnknown {
    fn from(value: &VideoTransformSphericalProjection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VideoTransformSphericalProjection> for ::windows::runtime::IInspectable {
    fn from(value: VideoTransformSphericalProjection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VideoTransformSphericalProjection> for ::windows::runtime::IInspectable {
    fn from(value: &VideoTransformSphericalProjection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VideoTransformSphericalProjection {}
unsafe impl ::std::marker::Sync for VideoTransformSphericalProjection {}
